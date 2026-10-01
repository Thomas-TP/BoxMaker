async (page) => {
  const assert = (ok, message) => { if (!ok) throw new Error(message); };
  await page.getByRole('button', {name: 'EN', exact: true}).click();
  await page.getByRole('button', {name: 'Reset settings', exact: true}).click();
  await page.getByRole('spinbutton', {name: 'Dimension A mm', exact: true}).fill('123');
  await page.getByText('Preview your box', {exact: true}).waitFor();
  await page.waitForFunction(() => JSON.parse(localStorage.getItem('boxmaker-draft-v5')).params.object[0] === 123);
  await page.reload();
  await page.getByText('Preview your box', {exact: true}).waitFor();
  assert(await page.getByRole('spinbutton', {name: 'Dimension A mm', exact: true}).inputValue() === '123', 'The valid draft must survive a restart');
  const upload = async (value) => page.locator('input[type=file]').setInputFiles({name: 'invalid.boxmaker.json', mimeType: 'application/json', buffer: Buffer.from(value)});
  await upload('{broken');
  await page.getByText('Could not open project: This file is not a valid JSON project.', {exact: true}).waitFor();
  assert(await page.getByRole('spinbutton', {name: 'Dimension A mm', exact: true}).inputValue() === '123', 'A corrupt project must leave the current draft intact');
  await upload(JSON.stringify({version: 99, params: {}}));
  await page.getByText('Could not open project: This project needs a newer version of Boxmaker. Update the app.', {exact: true}).waitFor();
  await page.getByRole('button', {name: 'Advanced settings', exact: true}).click();
  await page.getByRole('spinbutton', {name: 'Wall thickness mm', exact: true}).fill('0.1');
  await page.getByText('Wall thickness (mm) must be between 1.2 and 5.', {exact: true}).waitFor();
  await page.reload();
  await page.getByText('Preview your box', {exact: true}).waitFor();
  assert(await page.getByRole('spinbutton', {name: 'Dimension A mm', exact: true}).inputValue() === '123', 'Invalid edits must not overwrite the valid draft');
  await page.addInitScript(() => {
    const original = HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.getContext = function(type, ...args) {
      if (String(type).startsWith('webgl')) return null;
      return original.call(this, type, ...args);
    };
  });
  await page.reload();
  await page.getByText('Compatible preview · no graphics acceleration', {exact: true}).waitFor();
  const preview = page.locator('.three-view > svg');
  await page.waitForFunction(() => document.querySelectorAll('.three-view > svg path').length > 0);
  await preview.scrollIntoViewIfNeeded();
  const before = await preview.innerHTML();
  const bounds = await preview.boundingBox();
  await page.mouse.move(bounds.x + bounds.width * 0.4, bounds.y + bounds.height * 0.45);
  await page.mouse.down();
  await page.mouse.move(bounds.x + bounds.width * 0.65, bounds.y + bounds.height * 0.55, {steps: 8});
  await page.mouse.up();
  await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  assert(before !== await preview.innerHTML(), 'Dragging must rotate the software preview');
  for (const name of ['Closed', 'Print layout', 'Exploded view', 'Opening']) {
    await page.getByRole('button', {name, exact: true}).click();
    await page.waitForFunction(() => document.querySelectorAll('.three-view > svg path').length > 0);
  }
  await page.getByRole('button', {name: 'Updates', exact: true}).click();
  await page.getByText('Installed version · stable', {exact: true}).waitFor();
  await page.getByRole('checkbox', {name: /Join the beta/}).waitFor();
  await page.getByRole('button', {name: 'Close window', exact: true}).click();
  await page.getByRole('button', {name: 'Closed', exact: true}).click();
  await page.screenshot({path: 'output/playwright/boxmaker-v100-software-preview.png', fullPage: true});
  const complete = page.waitForEvent('download');
  await page.getByRole('button', {name: 'Export all 3 parts', exact: true}).click();
  await (await complete).saveAs('output/playwright/boxmaker-v100-software-export.3mf');
  return 'PASS: draft recovery, corrupt and future projects, localized range errors, invalid-edit recovery, software rendering/rotation/four views, stable label, and export without WebGL.';
}
