import { module, test } from 'qunit';

import { http, HttpResponse } from 'msw';

import { setupTest } from 'crates-io/tests/helpers';
import setupMsw from 'crates-io/tests/helpers/setup-msw';

module('Service | Playground', function (hooks) {
  setupTest(hooks);
  setupMsw(hooks);

  hooks.beforeEach(function () {
    this.playground = this.owner.lookup('service:playground');
  });

  test('`crates` are available if the request succeeds', async function (assert) {
    let crates = [
      { name: 'addr2line', version: '0.14.1', id: 'addr2line' },
      { name: 'adler', version: '0.2.3', id: 'adler' },
      { name: 'adler32', version: '1.2.0', id: 'adler32' },
      { name: 'ahash', version: '0.4.7', id: 'ahash' },
      { name: 'aho-corasick', version: '0.7.15', id: 'aho_corasick' },
      { name: 'ansi_term', version: '0.12.1', id: 'ansi_term' },
      { name: 'ansi_term', version: '0.11.0', id: 'ansi_term_0_11_0' },
    ];

    let response = HttpResponse.json({ crates });
    this.worker.use(http.get('https://play.rust-lang.org/meta/crates', () => response));

    await this.playground.loadCratesTask.perform();
    assert.deepEqual(this.playground.crates, crates);
  });

  test('loadCratesTask fails on HTTP error', async function (assert) {
    let error = HttpResponse.json({}, { status: 500 });
    this.worker.use(http.get('https://play.rust-lang.org/meta/crates', () => error));

    await assert.rejects(this.playground.loadCratesTask.perform());
    assert.notOk(this.playground.crates);
  });
});
