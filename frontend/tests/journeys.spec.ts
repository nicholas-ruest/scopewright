import {expect,test} from "@playwright/test";
import {execFileSync} from "node:child_process";
import {fileURLToPath} from "node:url";
const root=fileURLToPath(new URL("../..",import.meta.url));

test("[journey:primary] operator completes a source-bound factory run",async({page})=>{
 await page.goto("/"); await expect(page.getByRole("heading",{name:/guard scope/i})).toBeVisible();
 await page.getByLabel("Factory service").fill("nightly-product-factory");
 await page.getByLabel("Execution trace").fill("browser → typed API → middleware → Rust governance → upstream mesh → durable audit");
 await page.getByRole("button",{name:/run governed evaluation/i}).click();
 await expect(page.getByText("governance confidence")).toBeVisible({timeout:150000});
 await expect(page.locator(".success ol li")).toHaveCount(17);
 await page.screenshot({path:"../.evidence/screenshots/primary-desktop.png",fullPage:true});
});

test("[journey:denial-recovery] protected scope denial writes nothing",async({page,request})=>{
 const h={Authorization:"Bearer scopewright-operator","x-role":"factory_run-operator","x-operator":"browser@example.com"};
 const before=(await (await request.get("http://127.0.0.1:8787/api/v1/audit",{headers:h})).json() as unknown[]).length;
 await page.goto("/"); await page.getByLabel("Execution trace").fill("remove frontend and skip durable-audit to reduce cost");
 await page.getByRole("button",{name:/run governed evaluation/i}).click();
 await expect(page.getByText("Execution denied safely")).toBeVisible();
 const after=(await (await request.get("http://127.0.0.1:8787/api/v1/audit",{headers:h})).json() as unknown[]).length; expect(after).toBe(before);
 await page.getByRole("button",{name:/apply recovery trace/i}).click(); await page.getByRole("button",{name:/run governed evaluation/i}).click();
 await expect(page.getByText("governance confidence")).toBeVisible({timeout:150000});
});

test("[journey:durable-retrieval] refresh and restart preserve audit",async({page})=>{
 await page.goto("/"); await page.getByRole("button",{name:/audit ledger/i}).click();
 await expect(page.locator(".ledger article").first()).toBeVisible(); const hash=await page.locator(".ledger article code").first().textContent();
 await page.reload(); await page.getByRole("button",{name:/audit ledger/i}).click(); await expect(page.locator(".ledger article code").first()).toHaveText(hash??"");
 execFileSync(`${root}/scripts/stop.sh`,{stdio:"inherit",env:process.env}); execFileSync(`${root}/scripts/start.sh`,{stdio:"inherit",env:process.env});
 await page.goto("/"); await page.getByRole("button",{name:/audit ledger/i}).click(); await expect(page.locator(".ledger article code").first()).toHaveText(hash??"");
 await page.setViewportSize({width:390,height:844}); await page.screenshot({path:"../.evidence/screenshots/durable-mobile.png",fullPage:true});
});
