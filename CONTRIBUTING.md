# Contributing

All contributions, be it big or small, are encouraged and any help with the project is greatly appreciated.

## I want to contribute
### Boring legal nonsense
By creating a pull request to the project you fully agree that you have the rights to distribute the code in question and that the code, if licensed, can exist under the MIT license.
### Feature requests
If you have a feature request, please check if it isn't already present in the [backlog](https://github.com/users/joshniemela/projects/5). If it isn't, then either open up an issue or send the request to [Josh Niemelä](mailto:josh@jniemela.dk).
### Pull requests
Anything goes, but it is expected that a pull request should solve some particular issue in the active issues, or something in the backlog (make a feature request if your contribution doesn't fit the aforementioned). This pull request is expected to contain code that has been run through its respective formatter (Black for Python, Cljfmt for Clojure, (To come) for TS/Svelte).

### Running the project
The project contains a .env file which can be set to development or production, developmenet is the one that should be used. The individual components of the project can be run collectively using `docker-compose` or individually (`lein run` for Clojure, `pipenv run` for Python, `npm run dev` for TS/Svelte).

## Bugs
If you've found a bug or something that isn't intuitive in the user interface:
* Ensure this isn't already a known bug by looking at the [issues](https://github.com/joshniemela/disproject/issues).
* Try to replicate the unexpected behaviour.
* Please include the OS, Browser and other useful information in the bug report to make it easier to narrow it down.
* Write an issue about the problem, eventually possible solutions to the problem.
* Lastly, feel welcome to assign yourself to fixing the problem or tagging someone who might be able to fix it.

