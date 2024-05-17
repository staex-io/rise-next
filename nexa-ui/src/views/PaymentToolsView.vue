<script setup>
import CopyButton from '@/components/CopyButton.vue'
</script>

<script>
import router from '@/router'
const API_URL = 'https://egw.int.paymenttools.net/api/v2'
export default {
    data() {
        return {
            error: '',
            loaded: false,
        }
    },
    mounted() {
        fetch('/v1/session', {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        })
            .then(async (response) => {
                switch (response.status) {
                    case 201:
                        this.error = ''
                        const session = await response.json()
                        console.log('session id ' + session.id)
                        this.initPaymentToolsSession(session.id)
                        break
                    default:
                        this.error = await responseToError(response)
                        break
                }
            })
            .catch((error) => {
                this.error = error
            })
    },
    methods: {
        initPaymentToolsSession(sessionId) {
            const dropIn = new Paymenttools.DropIn('drop-in-container', {
                sessionId: sessionId,
                language: 'en',
                callback: (status) => {
                    if (status === 'SUCCEEDED') {
                        router.push({ name: 'photo' })
                    }
                },
            })
            this.loaded = true
        },
    },
}
</script>

<template>
    <p class="card-data">
        <b>Card number: </b><span id="card-number">4999 9999 9999 0011</span>
        &nbsp;<CopyButton id="card-number" /><br />
        <b>Expiry date: </b><span id="card-expiry">03/30</span>
        <!--&nbsp;<CopyButton id="card-expiry" />--><br />
        <b>CVC: </b><span id="card-cvc">737</span>&nbsp;<CopyButton id="card-cvc" />
    </p>
    <div id="drop-in-container">
        <div v-if="!loaded" id="loader-container">Please wait…<br/><span class="loader" /></div>
    </div>
    <div class="button-container">
        <a href="/photo" class="button-link-large">Buy!</a>
    </div>
</template>

<style scoped>
.card-data {
    font-size: normal;
}

.card-data button {
    padding: 10px !important;
    font-size: 12pt !important;
}

.loader {
    width: 32px !important;
    height: 32px !important;
}

#loader-container {
    text-align: center;
    margin: 2rem;
}
</style>
