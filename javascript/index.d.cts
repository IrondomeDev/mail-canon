export type Provider = 'custom' | 'gmail' | 'microsoft' | 'icloud' | 'proton' | 'yahoo';
export interface EmailIdentity {
  readonly domain: string;
  readonly canonical: string;
  readonly provider: Provider;
}
export class InvalidEmail extends Error {}
export function normalize(email: string): EmailIdentity;
export function canonicalize(email: string): string;
/** Normalize one bare address; reject wrappers, whitespace and cleanup. */
export function normalizeStrict(email: string): EmailIdentity;
export function canonicalizeStrict(email: string): string;
