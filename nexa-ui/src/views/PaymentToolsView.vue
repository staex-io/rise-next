<script>
const API_URL = 'https://egw.int.paymenttools.net/api/v2'
export default {
    data() {
        return {}
    },
    mounted() {
        const request = {
            orderId: '9999',
            money: {
                amount: '1.00',
                currency: 'EUR',
            },
            transactionDateTime: new Date().toISOString(),
            captureMode: 'AUTO',
            redirectUrl: 'https://web3.staex.io/photo',
            transactionInitiator: 'CONSUMER',
            channel: 'ecom',
            useCase: 'booking',
        }
        fetch(`${API_URL}/session`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
                Authorization: 
            },
            body: JSON.stringify(request),
        })
        const dropIn = new Paymenttools.DropIn('drop-in-container', {
            sessionId: 'dbff4bc0-1d82-49df-b3f3-9fa433f8b83b',
            language: 'de',
            callback: (status) => {
                console.log(status)
                if (status === 'SUCCEEDED') {
                }
            },
        })
    },
}
</script>

<template>
    <div id="drop-in-container"></div>
    <div class="button-container">
        <a href="/photo" class="button-link-large">Get my photo anyway</a>
    </div>
</template>
