"""Static release workflow invariants and offline published-release provenance tests.

Run with: python3 -m unittest discover -s tests -p 'test_*.py'
"""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

import yaml

ROOT = Path(__file__).resolve().parents[1]
PUBLISH = ROOT / '.github/workflows/publish.yml'
CI = ROOT / '.github/workflows/ci.yml'
RESOLVE = ROOT / 'scripts/resolve-release.sh'
COMMIT = 'a' * 40
OTHER = 'b' * 40


class PublishWorkflowTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.workflow = yaml.safe_load(PUBLISH.read_text())
        cls.jobs = cls.workflow['jobs']

    def test_only_published_release_or_master_retry(self):
        triggers = self.workflow[True]  # PyYAML 1.1 parses the Actions `on` key as True.
        self.assertEqual(triggers['release']['types'], ['published'])
        self.assertEqual(set(triggers['workflow_dispatch']['inputs']), {'version', 'commit'})
        self.assertTrue(all(x['required'] for x in triggers['workflow_dispatch']['inputs'].values()))
        for job in ('metadata', 'acceptance'):
            self.assertIn("github.ref == 'refs/heads/master'", self.jobs[job]['if'])
        self.assertIn("github.event_name == 'release'", self.jobs['publish']['if'])
        self.assertIn('github.event.release.tag_name', self.jobs['metadata']['if'])
        self.assertIn('github.event.release.tag_name', self.jobs['acceptance']['if'])
        self.assertEqual(self.workflow['concurrency']['cancel-in-progress'], False)
        group = self.workflow['concurrency']['group']
        self.assertIn("github.event.release.tag_name || format('v{0}', inputs.version)", group)
        self.assertEqual(self.jobs['acceptance']['timeout-minutes'], 180)
        self.assertEqual(self.jobs['metadata']['steps'][3]['env']['RELEASE_EVENT_SHA'], '${{ github.sha }}')

    def test_publication_and_acceptance_are_separate(self):
        metadata, publish, accept, status = (self.jobs[x] for x in ('metadata', 'publish', 'acceptance', 'release-status'))
        self.assertEqual(publish['needs'], 'metadata')
        self.assertEqual(accept['needs'], ['metadata', 'publish'])
        self.assertIn("needs.publish.result == 'skipped'", accept['if'])
        self.assertIn("needs.publish.result == 'success'", accept['if'])
        self.assertIn("needs.metadata.result == 'success'", accept['if'])
        self.assertIn("needs.publish.result", status['steps'][1]['env']['PUBLISH_RESULT'])
        self.assertIn("needs.acceptance.result", status['steps'][1]['env']['ACCEPTANCE_RESULT'])
        self.assertIn('published, but', status['steps'][1]['run'])
        self.assertEqual(status['if'], 'always()')
        self.assertEqual(publish['environment'], 'release')
        self.assertEqual(accept['environment'], 'release')
        self.assertEqual(publish['permissions']['id-token'], 'write')
        self.assertEqual(accept['permissions'], {})
        self.assertIn('cargo publish', publish['steps'][-1]['run'])
        self.assertNotIn('cargo publish', str(accept))

    def test_verified_version_and_sha_reach_full_action(self):
        metadata, publish, accept = (self.jobs[x] for x in ('metadata', 'publish', 'acceptance'))
        self.assertEqual(metadata['outputs']['sha'], '${{ steps.resolve.outputs.sha }}')
        self.assertEqual(metadata['outputs']['version'], '${{ steps.resolve.outputs.version }}')
        self.assertEqual(metadata['steps'][1]['with']['ref'], '${{ github.workflow_sha }}')
        self.assertIn('cp ./scripts/validate-release-tag.sh', metadata['steps'][2]['run'])
        self.assertIn('bash "$RUNNER_TEMP/validate-release-tag.sh"', metadata['steps'][-1]['run'])
        self.assertEqual(publish['steps'][1]['with']['ref'], '${{ needs.metadata.outputs.sha }}')
        self.assertIn('git ls-remote', publish['steps'][2]['run'])
        self.assertIn('test "$sha" = "$VERIFIED_SHA"', publish['steps'][2]['run'])
        self.assertIn('bash ./scripts/validate-release-tag.sh', publish['steps'][2]['run'])
        self.assertEqual(publish['steps'][2]['env']['VERIFIED_VERSION'], '${{ needs.metadata.outputs.version }}')
        action = accept['steps'][-1]
        self.assertEqual(action['with']['suite'], 'full')
        self.assertEqual(action['with']['package-version'], '${{ needs.metadata.outputs.version }}')
        self.assertEqual(action['with']['source-sha'], '${{ needs.metadata.outputs.sha }}')
        self.assertRegex(action['uses'], r'^rudderlabs/github-action-sdk-e2e@[0-9a-f]{40}$')
        self.assertIn('exit 1', accept['steps'][1]['run'])  # draft cannot invoke unsupported action
        self.assertFalse(any('actions/checkout' in str(s) or 'scripts/' in str(s.get('run', ''))
                             for s in accept['steps']))
        token = accept['steps'][2]
        self.assertEqual(token['with']['repositories'], 'rudder-sdk-e2e-tests')
        self.assertEqual(token['with']['permission-actions'], 'write')
        self.assertNotIn('permission-contents', token['with'])
        self.assertTrue(any('crates-io-auth-action@' in str(s) for s in publish['steps']))


class CIWorkflowTest(unittest.TestCase):
    def test_unprivileged_pr_and_master_workflow_tests(self):
        ci = yaml.safe_load(CI.read_text())
        self.assertIn('pull_request', ci[True])
        self.assertIn('master', ci[True]['push']['branches'])
        job = ci['jobs']['release-workflow-tests']
        self.assertEqual(job['permissions'], {'contents': 'read'})
        self.assertFalse('environment' in job)
        commands = job['steps'][-1]['run']
        self.assertIn('PyYAML==6.0.3', commands)
        self.assertIn("unittest discover -s tests -p 'test_*.py'", commands)


class ResolveReleaseTest(unittest.TestCase):
    def run_resolve(self, event='workflow_dispatch', ref='refs/heads/master', version='2.0.1',
                    sha=COMMIT, event_sha=COMMIT, tag='v2.0.1', remote=None, release=None):
        with tempfile.TemporaryDirectory() as temp:
            bin_dir = Path(temp)
            (bin_dir / 'curl').write_text('#!/bin/sh\nprintf %s "$MOCK_RELEASE"\n')
            (bin_dir / 'git').write_text('#!/bin/sh\nprintf %s "$MOCK_REMOTE"\n')
            for binary in ('curl', 'git'):
                (bin_dir / binary).chmod(0o755)
            output = bin_dir / 'output'
            env = dict(os.environ, PATH=f'{temp}:{os.environ["PATH"]}', GITHUB_OUTPUT=str(output),
                       GITHUB_EVENT_NAME=event, GITHUB_REF=ref,
                       GITHUB_REPOSITORY='rudderlabs/rudder-sdk-rust', GH_TOKEN='fake',
                       INPUT_VERSION=version, INPUT_SHA=sha, RELEASE_EVENT_SHA=event_sha, RELEASE_TAG=tag,
                       RELEASE_PRERELEASE='false', RELEASE_DRAFT='false',
                       MOCK_RELEASE=release or '{"tag_name":"v2.0.1","draft":false,"prerelease":false}',
                       MOCK_REMOTE=remote if remote is not None else f'{COMMIT}\trefs/tags/v2.0.1\n')
            result = subprocess.run(['bash', str(RESOLVE)], capture_output=True, text=True, env=env)
            return result.returncode, output.read_text() if output.exists() else '', result.stderr

    def test_existing_release_and_matching_retry(self):
        rc, output, _ = self.run_resolve()
        self.assertEqual(rc, 0)
        self.assertEqual(output, f'tag=v2.0.1\nversion=2.0.1\nsha={COMMIT}\n')
        rc, output, _ = self.run_resolve(event='release', ref='refs/tags/v2.0.1', sha='')
        self.assertEqual(rc, 0)
        self.assertIn(f'sha={COMMIT}', output)
        remote = f'{OTHER}\trefs/tags/v2.0.1\n{COMMIT}\trefs/tags/v2.0.1^{{}}\n'
        self.assertEqual(self.run_resolve(remote=remote)[0], 0)  # annotated tag dereference
        self.assertEqual(self.run_resolve(event='release', ref='refs/tags/v2.0.1',
                                          sha='', remote=remote)[0], 0)

    def test_mismatched_or_untrusted_input_fails_closed(self):
        invalid = (
            {'ref': 'refs/heads/feature'}, {'ref': 'refs/pull/1/merge'},
            {'event': 'pull_request'}, {'event': 'release', 'ref': 'refs/tags/v2.0.0'},
            {'event': 'release', 'ref': 'refs/tags/v2.0.1', 'event_sha': OTHER},
            {'event': 'release', 'ref': 'refs/tags/v2.0.1', 'event_sha': ''},
            {'sha': OTHER}, {'sha': 'latest'}, {'version': 'latest'},
            {'version': '2.0.2'}, {'remote': ''},
            {'release': '{"tag_name":"v2.0.1","draft":true,"prerelease":false}'},
            {'remote': f'{OTHER}\trefs/tags/v2.0.1\n'},
        )
        for case in invalid:
            with self.subTest(case=case):
                rc, output, _ = self.run_resolve(**case)
                self.assertNotEqual(rc, 0)
                self.assertEqual(output, '')


if __name__ == '__main__':
    unittest.main()
