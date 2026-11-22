// Module: web/apps/signal-web/src/main.ts — Zoneless Angular v22+ Application Bootstrap
// Invariants: Strictly Zoneless (provideZonelessChangeDetection); zero Zone.js; binds route inputs cleanly.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
import { provideHttpClient } from "@angular/common/http";
import { provideZonelessChangeDetection } from "@angular/core";
import { bootstrapApplication } from "@angular/platform-browser";
import { provideRouter, withComponentInputBinding } from "@angular/router";
import { APP_ROUTES } from "./app/app.routes";
import { IdeShellComponent } from "./app/shell/ide-shell.component";

bootstrapApplication(IdeShellComponent, {
  providers: [
    provideZonelessChangeDetection(),
    provideHttpClient(),
    provideRouter(APP_ROUTES, withComponentInputBinding()),
  ],
}).catch((err: unknown) => {
  console.error("Signal bootstrap failed:", err);
});
