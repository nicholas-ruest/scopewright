export type CreateFactoryRun = { service:string; trace_summary:string; scope_pressure:number; token_budget:number };
export type FactoryRunResponse = { id:string; correlation_id:string; status:string; risk_score:number; stages:string[]; evidence_hash:string };
export type ErrorResponse = { code:string; message:string; correlation_id:string };
export type EvidenceRecord = { id:string; correlation_id:string; payload:string; previous_hash:string; hash:string };

const headers = (role="factory_run-operator"): HeadersInit => ({
  "content-type":"application/json",
  authorization:"Bearer scopewright-operator",
  "x-operator":"studio@example.com",
  "x-role":role
});

async function decode<T>(response:Response):Promise<T> {
  if (response.ok) return response.json() as Promise<T>;
  const body = await response.text();
  try { const error=JSON.parse(body) as ErrorResponse; throw new Error(error.message); }
  catch (error) { if (error instanceof SyntaxError) throw new Error(body || `HTTP ${response.status}`); throw error; }
}

export const api = {
  create(input:CreateFactoryRun, role?:string) {
    return fetch("/api/v1/factory_runs",{method:"POST",headers:headers(role),body:JSON.stringify(input)}).then(decode<FactoryRunResponse>);
  },
  audit() { return fetch("/api/v1/audit").then(decode<EvidenceRecord[]>); }
};
