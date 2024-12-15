import fasttext
import pymorphy2
from fasttext.FastText import _FastText

import nltk
nltk.download('punkt')
nltk.download('stopwords')

from nltk.tokenize import word_tokenize
from nltk.corpus import stopwords

stop = stopwords.words('russian')

from string import punctuation
punkt= [p for p in punctuation] + ["`", "``" ,"''", "'"]

lemmatizer = pymorphy2.MorphAnalyzer(lang='ru')


def tokenize(sent):
    try:
        sent = word_tokenize(sent)
        return [word for word in sent if word not in stop and word not in punkt]
    except:
        return []
    

def lemmatize(sent):
    try:
        return ' '.join([lemmatizer.normal_forms(word)[0] for word in sent])
    except:
        return ' '


def preprocess_sent(sent):
    return lemmatize(tokenize(sent))


model_path = 'D:\Rust-NLP-web-service\functions\optimized.model' # процесс обучения можно найти в vk-hse-classifier.ipynb
ft_model = _FastText(model_path=model_path)


def get_tag(text):
    result = ft_model.predict(preprocess_sent(text), k=1)[0][0].split('__')[2]
    return result