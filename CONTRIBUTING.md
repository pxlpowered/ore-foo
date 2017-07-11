Contributor Guidelines
===

Thank you for contributing to Nucleus! Please have a read through this document
and make sure you understand it all before contributing.

## Reporting an Issue, Requesting a Feature

When submitting an issue through GitHub Issues, please remember that we are volunteers, and as volunteers, we have
limited time to be able to help. We would love to spend as much time fixing your issues and implementing features
as possible. To that end, please keep the following in mind when reporting bugs and requesting features:

* Make the title of the report clear. "It's broken" does not help us much, "Request always returns an error" is more helpful and lets us see what is wrong just by looking at the list.

* When reporting a bug:
    * Describe how you reproduce the bug step by step
    * Explain what you expect to happen
    * Explain what actually happens
    * Include backtraces (using RUST_BACKTRACE=1)

* When describing a feature:
    * Tell us what you'd like to see
    * Tell us why you want it, and what the use case is
    * Give us an idea on how it should work

* If we need more information about a bug or a feature, we'll ask for clarification! We want to get the system right.

For more information about writing a bug report in general, have a look at [this page](http://www.chiark.greenend.org.uk/~sgtatham/bugs.html),
particularly the summary at the bottom.

## Pull Requests

If you'd like to write code for us, great, welcome aboard! We have a few things that you should be aware of.

### Developing Ore.

As Ore is designed for multiple backends, use of feature gates is an important
part of the implementing them.

### Code Style

We follow Rust formatting and [naming conventions]. You can install `rustfmt` and
run `cargo fmt` to help with formatting. Some things to note:

* Line endings
    * Use Unix line endings when committing (\n).
    * Windows users of git can do `git config --global core.autocrlf true` to let git convert them automatically.

* Column width
    * 100 characters
    * Feel free to wrap when it will help with readability

* Indentation
    * Use 4 spaces for indentations, do not use tabs.

* Deprecation
    * Do not deprecate content unless you have a replacement, or if the provided feature is removed.
    * Be prepared to provide justification for the deprecation.
    * When deprecating, provide the month and year when it was deprecated.

Note that this style guide is _not_ a hard and fast requirement, but please keep your code style sane and similar
to ours.

### Submitting your Pull Requests
In your PRs, please make sure you fulfill the following:

* Provide a justification for the change - is it a new feature or a fix to a bug?
* Before sending a pull request ensure that your branch is up to date with the branch you are targeting.
* Do not squash commits unless directed to do so, but please _rebase_ your changes on top of master when you feel your changes are ready to be submitted - _do not merge_. We will squash the commits in a way we feel logical.

### Attribution
Based on, and adapted from, [Nucleus] Contibuting Guidelines.

[naming conventions]: https://aturon.github.io/style/naming.html
[Nucleus]: https://github.com/NucleusPowered/Nucleus
