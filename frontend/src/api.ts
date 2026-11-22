export type RunInput={service:string;trace_summary:string;scope_pressure:number;token_budget:number};
export type RunResponse={id:string;correlation_id:string;status:string;decision:string;risk_score:number;stages:string[];evidence_hash:string};
const headers={'content-type':'application/json',authorization:'Bearer scopewright-operator','x-role':'factory_run-operator'};
async function decode<T>(r:Response):Promise<T>{
  const body=await r.text();
  let data:unknown=body;
  try{data=body?JSON.parse(body):null;}catch{/* plaintext middleware errors are intentional */}
  if(!r.ok){const message=typeof data==='object'&&data!==null&&'message' in data?String((data as {message:unknown}).message):String(data||`request failed ${r.status}`);throw new Error(message);}
  return data as T;
}
export async function createRun(input:RunInput):Promise<RunResponse>{return decode<RunResponse>(await fetch('/api/v1/factory_runs',{method:'POST',headers,body:JSON.stringify(input)}));}
export async function listAudit():Promise<Array<{id:string;correlation_id:string;payload:string;hash:string}>>{const r=await fetch('/api/v1/audit',{headers});if(!r.ok)throw new Error(`audit failed ${r.status}`);return r.json();}
export async function replay(id:string){const r=await fetch(`/api/v1/factory_runs/${encodeURIComponent(id)}`,{headers});if(!r.ok)throw new Error(`replay failed ${r.status}`);return r.json();}
