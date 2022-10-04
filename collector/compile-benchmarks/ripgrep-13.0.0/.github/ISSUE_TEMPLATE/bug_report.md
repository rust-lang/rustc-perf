---
name: Bug report
about: An issue with ripgrep or any of its crates (ignore, globset, etc.)
title: ''
labels: ''
assignees: ''
---


#### What version of ripgrep are you using?

Replace this text with the output of `rg --version`.

#### How did you install ripgrep?

If you installed ripgrep with snap and are getting strange file permission or
file not found errors, then please do not file a bug. Instead, use one of the
Github binary releases.

#### What operating system are you using ripgrep on?

Replace this text with your operating system and version.

#### Describe your bug.

Give a high level description of the bug.

#### What are the steps to reproduce the behavior?

If possible, please include both your search patterns and the corpus on which
you are searching. Unless the bug is very obvious, then it is unlikely that it
will be fixed if the ripgrep maintainers cannot reproduce it.

If the corpus is too big and you cannot decrease its size, file the bug anyway
and the ripgrep maintainers will help figure out next steps.

#### What is the actual behavior?

Show the command you ran and the actual output. Include the `--debug` flag in
your invocation of ripgrep.

If the output is large, put it in a gist: https://gist.github.com/

If the output is small, put it in code fences:

```
your
output
goes
here
```

#### What is the expected behavior?

What do you think ripgrep should have done?
