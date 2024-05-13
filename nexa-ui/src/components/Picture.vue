<script setup>
const props = defineProps([
    'src',
    'alt',
    'class',
    'preset',
    'width',
    'height',
    'dynamic',
])
const components = props.src.split('/')
const isSvg = props.src.match(/\.svg$/)
const src = components.join('/')
const originalSrc = props.src
const webpPath = src + '.webp'
const webpExists = false
const dynamic = 'dynamic' in props && props.dynamic != null

function getWidth() {
    if (dynamic) {
        return ''
    }
    if ('width' in props && props.width) {
        return props.width
    }
    if (!props.preset || props.preset === '') {
        return ''
    }
    switch (props.preset) {
        case '320x320':
            return 320
        case '360x240':
            return 360
        case '160x160':
            return 160
        default:
            return props.preset.split('x')[0]
    }
}

function getHeight() {
    if (dynamic) {
        return ''
    }
    if ('height' in props && props.height) {
        return props.height
    }
    if (!props.preset || props.preset === '') {
        return ''
    }
    switch (props.preset) {
        case '320x320':
            return 320
        case '360x240':
            return 240
        case '160x160':
            return 160
        default:
            return props.preset.split('x')[1]
    }
}

function getPreset() {
    if (!props.preset || props.preset === '') {
        return null
    }
    return props.preset
}

const width = getWidth()
const height = getHeight()
const imageClasses = (() => {
    const classes = []
    if (props.class && props.class !== '') {
        classes.push(props.class)
    }
    if (props.preset && props.preset !== '') {
        classes.push(`preset-${props.preset}`)
    }
    return classes.join(' ')
})()
</script>

<script>
export default {
    data() {
        return {
            isFullScreen: false,
            classes: '',
        }
    },
    methods: {
        toggleFullScreen(src, originalSrc, event) {
            if (!isInsideFigure(event.target)) {
                return
            }
            this.isFullScreen = !this.isFullScreen
            if (this.isFullScreen) {
                event.target.src = originalSrc
                this.classes = 'fullscreen'
                document.body.classList.add('disable-scrolling')
            } else {
                event.target.src = src
                this.classes = ''
                document.body.classList.remove('disable-scrolling')
            }
        },
    },
}

function isInsideFigure(element) {
    while (element != null) {
        if (element.tagName === 'FIGURE') {
            return true
        }
        element = element.parentNode
    }
    return false
}
</script>

<template>
    <picture
        @click="toggleFullScreen(src, originalSrc, $event)"
        :class="classes">
        <source
            v-if="webpExists"
            :srcset="webpPath"
            type="image/webp"
            :width="width"
            :height="height" />
        <img
            :src="src"
            :alt="props.alt"
            :class="imageClasses"
            :width="width"
            :height="height"
            loading="lazy" />
    </picture>
</template>
