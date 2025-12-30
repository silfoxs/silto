import { createApp } from 'vue'
import PopupView from './components/PopupView.vue'
import { createI18n } from 'vue-i18n'
import zhCN from './locales/zh-CN.json'
import enUS from './locales/en-US.json'
import './styles/index.css'

const savedLanguage = localStorage.getItem('language') || 'zh-CN'

const i18n = createI18n({
    legacy: false,
    locale: savedLanguage,
    fallbackLocale: 'zh-CN',
    messages: {
        'zh-CN': zhCN,
        'en-US': enUS,
    },
})

const app = createApp(PopupView)
app.use(i18n)
app.mount('#app')
