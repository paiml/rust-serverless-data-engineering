import random

def lambda_handler(event, context):
    message = event.get('message', '').strip().lower()
    
    if message == 'echo':
        response = random.choice(['dark ECHO', 'light ECHO'])
    else:
        response = f"You said: {message}. Say 'echo' to play the tunnel game!"
    
    return {
        'statusCode': 200,
        'body': response
    }
