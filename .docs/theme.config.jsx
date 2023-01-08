import { useRouter } from 'next/router'
import Footer from 'components/Footer'
import Logo from 'components/Logo'

export default {
    logo: <Logo />,
    project: {
        link: 'https://github.com/subtalegames/mimir',
    },
    docsRepositoryBase: 'https://github.com/subtalegames/mimir/blob/main/.docs',
    useNextSeoProps() {
        const { route } = useRouter()
        if (route !== '/') {
            return {
                titleTemplate: '%s – Mímir'
            }
        } else {
            return {
                titleTemplate: 'Mímir - Contextual query engine for dynamic video games',
            }
        }
    },
    footer: {
        text: <Footer />,
    },
}
