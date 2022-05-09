export function hello() {
    return "hello"
}

export function send() {
    const client = google.accounts.oauth2.initCodeClient({
        client_id: '839980808596-tq6nkmcik0nrohr079rj4vt5bdhvr15g.apps.googleusercontent.com',
        scope: 'https://www.googleapis.com/auth/calendar.readonly',
        ux_mode: 'popup',
        callback: (response) => {
            const xhr = new XMLHttpRequest();
            xhr.open('POST', code_receiver_uri, true);
            xhr.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
            // Set custom header for CRSF
            xhr.setRequestHeader('X-Requested-With', 'XmlHttpRequest');
            xhr.onload = function () {
                console.log('Auth code response: ' + xhr.responseText);
            };
            xhr.send('code=' + code);
        },
    });

    client.requestCode();
}