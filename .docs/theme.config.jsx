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
                titleTemplate: '%s – Mímir',
                description: 'Mímir is a contextual query engine (implemented in Rust) for video games with dynamic events (e.g. dialog, animations) driven by their current world\'s state.',
            }
        } else {
            return {
                titleTemplate: 'Mímir - Contextual query engine for dynamic video games',
                description: 'Mímir is a contextual query engine (implemented in Rust) for video games with dynamic events (e.g. dialog, animations) driven by their current world\'s state.',
            }
        }
    },
    footer: {
        text: <Footer />,
    },
}
