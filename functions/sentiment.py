from transformers import pipeline

model_sentiment = pipeline(model='MonoHime/rubert-base-cased-sentiment-new')

def sentiment(text):
    result = model_sentiment(text[:256])[0]['label']
    
    map_sentiment = {
        'NEGATIVE': 'Негативная',
        'NEUTRAL': 'Нейтральная',
        'POSITIVE': 'Позитивная'
    }
    
    return map_sentiment.get(result, 'Нейтральная')