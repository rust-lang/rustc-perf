import { module, test } from 'qunit';

import { http, HttpResponse } from 'msw';

import { setupTest } from 'crates-io/tests/helpers';
import setupMsw from 'crates-io/tests/helpers/setup-msw';

module('Adapter | crate', function (hooks) {
  setupTest(hooks);
  setupMsw(hooks);

  test('findRecord requests are coalesced', async function (assert) {
    let _foo = this.db.crate.create({ name: 'foo' });
    this.db.version.create({ crate: _foo });
    let _bar = this.db.crate.create({ name: 'bar' });
    this.db.version.create({ crate: _bar });

    // if request coalescing works correctly, then this regular API endpoint
    // should not be hit in this case
    let error = HttpResponse.json({}, { status: 500 });
    this.worker.use(http.get('/api/v1/crates/:crate_name', () => error));

    let store = this.owner.lookup('service:store');

    let [foo, bar] = await Promise.all([store.findRecord('crate', 'foo'), store.findRecord('crate', 'bar')]);
    assert.strictEqual(foo?.name, 'foo');
    assert.strictEqual(bar?.name, 'bar');
  });

  test('findRecord requests do not include versions by default', async function (assert) {
    let _foo = this.db.crate.create({ name: 'foo' });
    this.db.version.create({ crate: _foo, num: '0.0.1' });
    this.db.version.create({ crate: _foo, num: '0.0.2' });
    this.db.version.create({ crate: _foo, num: '0.0.3' });

    let versions = this.db.version.getAll().reverse();
    let default_version = versions.find(it => it.num === '0.0.3');

    let store = this.owner.lookup('service:store');

    let foo = await store.findRecord('crate', 'foo');
    assert.strictEqual(foo?.name, 'foo');

    // Only `defaul_version` should be loaded
    let versionsRef = foo.hasMany('versions');
    assert.deepEqual(versionsRef.ids(), [`${default_version.id}`]);

    await versionsRef.load();
    assert.deepEqual(
      versionsRef.ids(),
      versions.map(it => `${it.id}`),
    );
  });
});
