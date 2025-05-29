import { expect, test } from '@/e2e/helper';

test.describe('Acceptance | Logout', { tag: '@acceptance' }, () => {
  test('successful logout', async ({ page, msw }) => {
    let user = msw.db.user.create({ name: 'John Doe' });
    await msw.authenticateAs(user);

    await page.goto('/crates');
    await expect(page).toHaveURL('/crates');

    const menu = page.locator('[data-test-user-menu]');
    await expect(menu.locator('[data-test-toggle]')).toHaveText('John Doe');

    await menu.locator('[data-test-toggle]').click();
    await menu.locator('[data-test-logout-button]').click();

    await page.waitForURL('/');
  });
});
