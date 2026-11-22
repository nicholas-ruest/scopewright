import { expect, test } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
const root=fileURLToPath(new URL("../..",import.meta.url));

test("[journey:primary] operator completes a source-bound factory_run",async({page})=>{
  await page.goto("/");
  await expect(page.getByRole("heading",{name:/rehearse the outage/i})).toBeVisible();
  await page.getByLabel("Service").fill("checkout-edge");
  await page.getByLabel("Failure trace_summary").fill("regional message-bus isolation");
  await page.getByRole("button",{name:/run controlled factory_run/i}).click();
  await expect(page.getByText("FactoryRun completed")).toBeVisible({timeout:120_000});
  await expect(page.getByTestId("result")).toContainText("17 accountable stages");
  await page.screenshot({path:"../.evidence/screenshots/primary-desktop.png",fullPage:true});
});

test("[journey:denial-recovery] policy denial writes nothing and recovery succeeds",async({page,request})=>{
  const before=(await (await request.get("http://127.0.0.1:8787/api/v1/audit")).json() as unknown[]).length;
  await page.goto("/");
  await page.getByLabel("Operator role").selectOption("observer");
  await page.getByRole("button",{name:/run controlled factory_run/i}).click();
  await expect(page.getByRole("alert")).toContainText("policy denied");
  const after=(await (await request.get("http://127.0.0.1:8787/api/v1/audit")).json() as unknown[]).length;
  expect(after).toBe(before);
  await page.getByRole("button",{name:/restore operator access/i}).click();
  await page.getByRole("button",{name:/run controlled factory_run/i}).click();
  await expect(page.getByText("FactoryRun completed")).toBeVisible({timeout:120_000});
});

test("[journey:durable-retrieval] refresh renders saved hash-chain evidence",async({page})=>{
  execFileSync(`${root}/scripts/stop.sh`,{stdio:"inherit"});
  execFileSync(`${root}/scripts/start.sh`,{stdio:"inherit"});
  await page.goto("/");
  await page.getByRole("button",{name:/evidence/i}).click();
  await expect(page.getByTestId("evidence-card").first()).toBeVisible();
  const correlation=await page.getByTestId("evidence-card").first().locator("dd").first().textContent();
  await page.reload();
  await page.getByRole("button",{name:/evidence/i}).click();
  await expect(page.getByTestId("evidence-card").first()).toContainText(correlation ?? "missing");
  await page.setViewportSize({width:390,height:844});
  await page.screenshot({path:"../.evidence/screenshots/durable-mobile.png",fullPage:true});
});
