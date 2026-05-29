## /admin
### GET
- /dashboard - admin dashboard
- /password - to change password

### POST
- /password - to actually change the password
- /login - clears Redis state

## /subscriptions
### POST
- / - subscribe to the newsletter - not yet confirmed

### GET
#### /confirm
Confirms the subscription. Note that the subscription_token is stored in a separate database with the user_id.

### GET
- /login - login form
- /health_check - health check
- /newsletters - 

