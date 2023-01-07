import {useRouter} from "next/router";

export default {
    logo: <span>Mímir</span>,
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
}
