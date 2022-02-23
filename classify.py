'''
Seeing if we can use machine learning to distinguish a wordle word from the
complete list of words.

This model achieves around 81% accuracy
'''
import pandas as pd
from sklearn.model_selection import train_test_split
from xgboost.sklearn import XGBClassifier
with open("all_words.txt", "r") as aw:
    a = aw.read().strip()
    all_words = a.split(' ')

with open("wordle_words.txt", "r") as ww:
    w = ww.read().strip()
    wordle_words = set(w.split(' '))

df = pd.DataFrame({
    'IsWord': [int(w in wordle_words) for w in all_words],
    0: [ord(w[0]) for w in all_words],
    1: [ord(w[1]) for w in all_words],
    2: [ord(w[2]) for w in all_words],
    3: [ord(w[3]) for w in all_words],
    4: [ord(w[4]) for w in all_words],
})
print(df.head())
y = df['IsWord']
X = df.loc[:, df.columns != 'IsWord']
X_train, X_valid, y_train, y_valid = train_test_split(X, y, test_size=0.2)
model = XGBClassifier(n_estimators = 1000, learning_rate = 0.05,  use_label_encoder=False, verbose=False)
model.fit(X_train, y_train, early_stopping_rounds = 5, eval_set=[(X_valid, y_valid)])
preds = model.predict(X_valid)
print((preds == y_valid.values).sum()/preds.shape[0])
