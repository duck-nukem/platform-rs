# Security decisions record

## Absence of CSRF cookie

After re-thinking the threat CSRF poses(1) the decision is to move to session cookies
using the SameSite: Strict configuration. This paired with disallowing modifications via
GET requests should be sufficient to protect against this kind of attack.

(1) https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html#samesite-cookie-attribute

## Test

1. Run the application
2. Log in to the application
3. Open poc.html in this directory

There should be a broken image. If you inspect the network tab it should point
to `/login` and not `/greet`

4. Open the link "External link"

It should ask you to log in. This is an effect of `SameSite: strict`. If we ever
want to relax the rules, we could opt for `SameSite: Lax`, while still being
fairly protected against CSRF attacks
