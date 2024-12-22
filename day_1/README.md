
## The Story

It's 1st December and the countdown has just begun. The elves are busy preparing for Christmas and Santa is busy checking the list of children who have been good this year. It was supposed to be a smooth day until all of a sudden two of Santa's elves burst into Santa's office with a problem.

“Santa!” one of the elves shouted. “The code won’t compile! We’ve hit a wall, and it’s all Rust’s fault!”

Santa, sipping his triple-shot peppermint latte, raised an eyebrow. “Rust’s fault? Or your fault?”

“It’s the ownership rules!” the other elf blurted. “I think we violated them, we’re used to Python, where variables just... work. Look at this!”

## Your Mission

The elves have tried their best but the code doesn't compile.

You need to find a way to make the code compile without changing the `attach_message_to_present` function.

## Hints

- Use `clone()` to create a clone of the `gift_message` and pass it to the function.
