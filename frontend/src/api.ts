export type RunInput={service:string;trace_summary:string;scope_pressure:number;token_budget:number};
export type RunResponse={id:string;correlation_id:string;status:string;decision:string;risk_score:number;stages:string[];evidence_hash:string};
const headers={'content-type':'application/json',authorization:'Bearer scopewright-operator'};
export async function createRun(input:RunInput):Promise<RunResponse>{const r=await fetch('/api/v1/factory_runs',{method:'POST',headers,body:JSON.stringify(input)});const data=await r.json();if(!r.ok)throw new Error(data.message||`request failed ${r.status}`);return data;}
export async function listAudit():Promise<Array<{id:string;correlation_id:string;payload:string;hash:string}>>{const r=await fetch('/api/v1/audit',{headers});if(!r.ok)throw new Error(`audit failed ${r.status}`);return r.json();}
export async function replay(id:string){const r=await fetch(`/api/v1/factory_runs/${encodeURIComponent(id)}`,{headers});if(!r.ok)throw new Error(`replay failed ${r.status}`);return r.json();}
