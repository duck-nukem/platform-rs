# Session

As an initial implementation basic sessions were chosen (at the moment: signed, but not encrypted).

This decision was made to be aligned with the philosophy of the application and strive for
simplicity, performance, and security.

Sessions are currently only stored in cookies, which makes invalidation impossible, but
other industry-standard (and widely used) solutions; such as JWT tokens also suffer
from the same problem, therefore it's probably good enough for us.

Initially sessions were stored in the database, but that resulted in a massive slowdown (1k rps)
compared to cookies only (6k rps). The added security isn't significant, so the decision
is to move away from db-based sessions.

However sessions live for 10 minutes only (refreshed for another 10 minutes on every interaction; configurable), 
are http only, and should be set to be secure on production environments. 
Applying all of this should make it fairly difficult to get ahold of someone else's cookie.
