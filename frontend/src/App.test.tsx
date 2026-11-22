import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { App } from "./App";
describe("Scopewright studio",()=>{
  it("renders a domain-specific factory_run composer",()=>{
    vi.stubGlobal("fetch",vi.fn().mockResolvedValue({ok:true,json:async()=>[]}));
    render(<App/>);
    expect(screen.getByRole("heading",{name:/evaluate factory trace/i})).toBeInTheDocument();
    expect(screen.getByRole("button",{name:/run governed evaluation/i})).toBeInTheDocument();
  });
});
