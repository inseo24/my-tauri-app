document.getElementById('send-btn').addEventListener('click', () => {
    const userInput = document.getElementById('user-input').value;
    if (!userInput.trim()) return;

    addMessageToChat('User', userInput);
    window.__TAURI__.invoke('chat', {
        payload: { messages: [{ role: 'user', content: userInput }] }
    })
    .then(response => {
        console.log('ChatGPT response:', response);
        if (response.choices && response.choices.length > 0) {
            addMessageToChat('AI', response.choices[0].message.content);
        }
    })
    .catch(error => {
        console.error('Failed to send message to ChatGPT:', error);
    });

    document.getElementById('user-input').value = '';
});

function addMessageToChat(sender, message) {
    const chatMessages = document.getElementById('chat-messages');
    const messageElement = document.createElement('li');
    messageElement.textContent = `${sender}: ${message}`;
    chatMessages.appendChild(messageElement);
}