export const handler = async (event) => {
    const message = (event.body ? JSON.parse(event.body).message :
                     event.message || 
                     event.queryStringParameters?.message || 
                     event || '').trim().toLowerCase();

    const response = message === 'echo'
        ? Math.random() < 0.5 ? '🌑 dark ECHO' : '💡 light ECHO'
        : `You said: ${message}. Say 'echo' to play the tunnel game!`;

    return {
        statusCode: 200,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: response })
    };
};
