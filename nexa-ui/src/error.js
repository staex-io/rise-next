export async function responseToError(response) {
    let text = await response.text()
    if (text.length === 0) {
        text = response.status.toString()
        if ('statusText' in response) {
            const value = response.statusText.trim()
            if (value !== '') {
                text += ' ' + value
            }
        }
    }
    return text
}
