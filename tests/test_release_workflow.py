import json
import os
import subprocess
import tempfile
import unittest
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[1]


def workflow(path):
    return yaml.load((ROOT / path).read_text(), Loader=yaml.BaseLoader)


def step(job, name):
    return next(item for item in job["steps"] if item["name"] == name)


class ReleaseWorkflowTest(unittest.TestCase):
    def test_publication_uses_exact_event_commit_and_stable_version_before_upload(self):
        publish = workflow(".github/workflows/publish.yml")
        self.assertEqual(publish["on"], {"release": {"types": ["published"]}})
        self.assertEqual(publish["concurrency"]["cancel-in-progress"], "false")
        job = publish["jobs"]["publish"]
        self.assertIn("!github.event.release.prerelease", job["if"])
        self.assertIn("refs/tags/{0}", job["if"])
        steps = job["steps"]
        checkout = step(job, "Check out repository")
        self.assertEqual(checkout["with"]["ref"], "${{ github.sha }}")
        validation = step(job, "Validate immutable release commit, tag, and stable crate version")
        self.assertIn('test "$sha" = "$GITHUB_SHA"', validation["run"])
        self.assertLess(steps.index(validation), steps.index(step(job, "Authenticate with crates.io")))
        self.assertLess(steps.index(validation), steps.index(step(job, "Publish crate")))

    def test_acceptance_and_notification_wait_for_upload_without_retry_reannouncement(self):
        publish = workflow(".github/workflows/publish.yml")["jobs"]
        acceptance = publish["published-e2e"]
        self.assertEqual(acceptance["needs"], "publish")
        self.assertEqual(acceptance["with"]["package-version"], "${{ github.event.release.tag_name }}")
        notify = publish["notify-release"]
        self.assertEqual(notify["needs"], ["publish", "published-e2e"])
        self.assertIn("always()", notify["if"])
        self.assertIn("github.event_name == 'release'", notify["if"])
        self.assertIn("refs/tags/{0}", notify["if"])
        self.assertNotIn("needs.publish.result", notify["if"])
        self.assertEqual(notify["permissions"], {})
        retry = workflow(".github/workflows/published-sdk-e2e.yml")
        self.assertIn("workflow_dispatch", retry["on"])
        validator = retry["jobs"]["validate"]
        self.assertNotIn("steps", validator)
        self.assertRegex(validator["uses"], r"^rudderlabs/github-action-sdk-e2e/\.github/workflows/published-sdk-e2e\.yml@[0-9a-f]{40}$")
        self.assertEqual(validator["with"]["package-version"], "${{ inputs.package-version || github.ref_name }}")
        self.assertEqual(validator["secrets"], {"RELEASE_PRIVATE_KEY": "${{ secrets.RELEASE_PRIVATE_KEY }}"})
        self.assertNotIn("slackapi/slack-github-action", str(retry))
        self.assertNotIn("cargo publish", str(retry))

    def test_stable_tag_validator_rejects_prerelease_and_mismatch(self):
        with tempfile.TemporaryDirectory() as temp:
            cargo = Path(temp) / "cargo"
            cargo.write_text("#!/bin/sh\nprintf '%s\\n' '{\"packages\":[{\"version\":\"1.2.3\"}]}'\n")
            cargo.chmod(0o755)
            env = os.environ.copy()
            env["PATH"] = f"{temp}:{env['PATH']}"
            for tag, expected in [("v1.2.3", 0), ("v1.2.3-rc.1", 1), ("v1.2.4", 1)]:
                with self.subTest(tag=tag):
                    result = subprocess.run(
                        ["bash", "scripts/validate-release-tag.sh"],
                        cwd=ROOT,
                        env={**env, "RELEASE_TAG": tag},
                        capture_output=True,
                        text=True,
                    )
                    self.assertEqual(result.returncode == 0, expected == 0, result.stderr)

    def test_slack_root_and_thread_contract_for_final_outcomes(self):
        job = workflow(".github/workflows/publish.yml")["jobs"]["notify-release"]
        steps = job["steps"]
        classify = step(job, "Classify final release outcome")["run"]
        prepare = step(job, "Prepare short release message and detail thread")["run"]
        root_action = step(job, "Post release status")
        thread_action = step(job, "Post release checks in thread")
        self.assertEqual(root_action["uses"], thread_action["uses"])
        self.assertIn("slackapi/slack-github-action@dcb1066f776dd043e64d0e8ba94ca15cc7e1875d", root_action["uses"])
        self.assertLess(steps.index(root_action), steps.index(thread_action))
        self.assertIn("steps.root.outputs.ts", step(job, "Require release thread timestamp")["env"]["ROOT_TS"])
        self.assertIn("SLACK_BOT_TOKEN", step(job, "Require existing Slack configuration")["run"])
        self.assertIn("SLACK_RELEASE_CHANNEL_ID", step(job, "Require existing Slack configuration")["run"])
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp) / "out"
            summary = Path(temp) / "summary"
            env = {
                **os.environ,
                "RUNNER_TEMP": temp,
                "GITHUB_OUTPUT": str(output),
                "GITHUB_STEP_SUMMARY": str(summary),
                "GITHUB_REPOSITORY": "rudderlabs/rudder-sdk-rust",
                "GITHUB_RUN_ID": "123",
                "RELEASE_TAG": "v1.2.3",
                "SLACK_CHANNEL_ID": "C123",
            }
            outcomes = [
                ("success", "success", "accepted"),
                ("success", "failure", "published; validation failed or incomplete"),
                ("success", "cancelled", "published; validation failed or incomplete"),
                ("success", "skipped", "published; validation failed or incomplete"),
                ("failure", "skipped", "publication failed or unconfirmed"),
                ("cancelled", "skipped", "publication failed or unconfirmed"),
            ]
            for publish_result, acceptance_result, expected in outcomes:
                with self.subTest(publish=publish_result, acceptance=acceptance_result):
                    output.write_text("")
                    summary.write_text("")
                    result_env = {**env, "PUBLISH_RESULT": publish_result, "ACCEPTANCE_RESULT": acceptance_result}
                    subprocess.run(["bash", "-e", "-o", "pipefail", "-c", classify], env=result_env, check=True)
                    values = dict(line.split("=", 1) for line in output.read_text().splitlines())
                    subprocess.run(
                        ["bash", "-e", "-o", "pipefail", "-c", prepare],
                        env={**result_env, "HEADLINE": values["headline"], "VERDICT": values["verdict"]},
                        check=True,
                    )
                    root = json.loads((Path(temp) / "release-root.json").read_text())
                    detail = json.loads((Path(temp) / "release-detail.json").read_text())
                    self.assertIn(expected, root["text"])
                    self.assertIn(expected, summary.read_text().lower())
                    self.assertRegex(root["text"], r"Rust SDK v1\.2\.3 .* \d{4}-\d{2}-\d{2}\nCC: <!subteam\^")
                    self.assertIn("<!subteam^S0555JBV36D> <!subteam^S03SHJ20350>", root["text"])
                    self.assertNotIn("https://", root["text"])
                    self.assertIn("https://crates.io/crates/rudderanalytics/1.2.3", detail["text"])
                    self.assertIn(f"*Publication job:* {publish_result}", detail["text"])
                    self.assertIn(f"*Acceptance job:* {acceptance_result}", detail["text"])
                    self.assertIn("*Run:*", detail["text"])
                    self.assertFalse(root["unfurl_links"])
                    self.assertFalse(root["unfurl_media"])
                    self.assertFalse(detail["unfurl_links"])
                    self.assertFalse(detail["unfurl_media"])
                    self.assertFalse(detail["reply_broadcast"])
                    self.assertEqual("accepted" in root["text"], publish_result == acceptance_result == "success")
                    if publish_result != "success":
                        self.assertIn("Registry lookup (publication unconfirmed)", detail["text"])
                        self.assertIn("Registry state is unconfirmed", summary.read_text())
                        self.assertNotIn("Crate published", summary.read_text())

    def test_missing_slack_configuration_is_reported_without_sending(self):
        job = workflow(".github/workflows/publish.yml")["jobs"]["notify-release"]
        check = step(job, "Require existing Slack configuration")["run"]
        with tempfile.TemporaryDirectory() as temp:
            summary = Path(temp) / "summary"
            result = subprocess.run(
                ["bash", "-e", "-o", "pipefail", "-c", check],
                env={**os.environ, "SLACK_BOT_TOKEN": "", "SLACK_CHANNEL_ID": "", "GITHUB_STEP_SUMMARY": str(summary)},
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("Missing SLACK_BOT_TOKEN or SLACK_RELEASE_CHANNEL_ID", summary.read_text())

    def test_nonstable_release_does_not_prepare_a_notification(self):
        job = workflow(".github/workflows/publish.yml")["jobs"]["notify-release"]
        classify = step(job, "Classify final release outcome")["run"]
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp) / "out"
            summary = Path(temp) / "summary"
            result = subprocess.run(
                ["bash", "-e", "-o", "pipefail", "-c", classify],
                env={
                    **os.environ,
                    "RELEASE_TAG": "v1.2.3-rc.1",
                    "PUBLISH_RESULT": "failure",
                    "ACCEPTANCE_RESULT": "skipped",
                    "GITHUB_OUTPUT": str(output),
                    "GITHUB_STEP_SUMMARY": str(summary),
                },
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(output.exists())
            self.assertIn("no release notification was sent", summary.read_text())


if __name__ == "__main__":
    unittest.main()
