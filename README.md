# schedulii

schedulii is an application that allows you to sync your calendars with other people to best figure out when everyone is free. 

## Tools
### Backend
The application runs on a rust backend using the axum web framework. 

### Frontend
The UI components of the app are built out in Svelte. 

## Setup
The application is configured to be easy to run on local. You just need to run `make build_and_run` to start the application. 

Any PR to the repo requires CI checks to pass and at least one approval from a reviewer before it can be merged in. All PR titles must follow the conventional commits standard. All tests (both unit and integration) must pass before review is requested. 

If you wish to contribute to a feature that requires access to a third party calendar provider (Outlook or Google Calendar), please contact the maintainers to get access. 


