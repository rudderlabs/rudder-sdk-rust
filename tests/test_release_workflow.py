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
        self.assertIn("needs.publish.result == 'success'", notify["if"])
        self.assertEqual(notify["permissions"], {})
        retry = workflow(".github/workflows/published-sdk-e2e.yml")
        self.assertIn("workflow_dispatch", retry["on"])
        self.assertEqual(retry["jobs"]["validate"]["runs-on"], "ubuntu-latest")
        self.assertEqual(step(retry["jobs"]["validate"], "Validate the exact public package, delivery, and cleanup")["with"]["suite"], "full")
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

    def test_slack_root_and_thread_contract_for_both_outcomes(self):
        job = workflow(".github/workflows/publish.yml")["jobs"]["notify-release"]
        steps = job["steps"]
        prepare = step(job, "Prepare short release message and detail thread")["run"]
        root_action = step(job, "Post release status")
        thread_action = step(job, "Post release checks in thread")
        self.assertEqual(root_action["uses"], thread_action["uses"])
        self.assertIn("slackapi/slack-github-action@dcb1066f776dd043e64d0e8ba94ca15cc7e1875d", root_action["uses"])
        self.assertLess(steps.index(root_action), steps.index(thread_action))
        self.assertIn("steps.root.outputs.ts", step(job, "Require release thread timestamp")["env"]["ROOT_TS"])
        self.assertIn("SLACK_BOT_TOKEN", step(job, "Require existing Slack configuration")["run"])
        self.assertIn("SLACK_RELEASE_CHANNEL_ID", step(job, "Require existing Slack configuration")["run"])
        self.assertIn("Crate published; validation failed or incomplete", step(job, "Require existing Slack configuration")["run"])
        with tempfile.TemporaryDirectory() as temp:
            env = {
                **os.environ,
                "RUNNER_TEMP": temp,
                "GITHUB_REPOSITORY": "rudderlabs/rudder-sdk-rust",
                "GITHUB_RUN_ID": "123",
                "RELEASE_TAG": "v1.2.3",
                "SLACK_CHANNEL_ID": "C123",
            }
            for result in ("success", "failure", "skipped"):
                with self.subTest(result=result):
                    subprocess.run(["bash", "-e", "-o", "pipefail", "-c", prepare], env={**env, "ACCEPTANCE_RESULT": result}, check=True)
                    root = json.loads((Path(temp) / "release-root.json").read_text())
                    detail = json.loads((Path(temp) / "release-detail.json").read_text())
                    self.assertRegex(root["text"], r"Rust SDK v1\.2\.3 .* \d{4}-\d{2}-\d{2}\nCC: <!subteam\^")
                    self.assertIn("<!subteam^S0555JBV36D> <!subteam^S03SHJ20350>", root["text"])
                    self.assertNotIn("https://", root["text"])
                    self.assertIn("https://crates.io/crates/rudderanalytics/1.2.3", detail["text"])
                    self.assertIn("*Publication:* success", detail["text"])
                    self.assertIn("*Run:*", detail["text"])
                    self.assertFalse(root["unfurl_links"])
                    self.assertFalse(root["unfurl_media"])
                    self.assertFalse(detail["unfurl_links"])
                    self.assertFalse(detail["unfurl_media"])
                    self.assertFalse(detail["reply_broadcast"])
                    self.assertEqual("accepted" in root["text"], result == "success")
                    if result != "success":
                        self.assertIn("published; validation failed or incomplete", root["text"])


if __name__ == "__main__":
    unittest.main()
