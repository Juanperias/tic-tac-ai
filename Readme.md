# Tic tac ai

fairly simple ia model with the objective of predicting the winner of a tic tac toe game

Dataset credits to: https://www.kaggle.com/datasets/ulrikthygepedersen/tic-tac-toe

# Getting started

Let's start by training the model with:

```
cargo r --train --file tic_tac_toc.csv --dist test.json
```

And now let's make a simple prediction.

```
cargo r -- run ***/***/*** --file test.json
```

Now you must know this before making your first prediction
x = x square
o = o square
b = this is equal to an empty space
