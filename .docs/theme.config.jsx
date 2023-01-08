import {useRouter} from "next/router";

export default {
    logo: <span className="font-black text-lg flex items-center">Mímir <span className="ml-4 font-bold px-3 py-1 text-sm rounded bg-gradient-to-tr from-indigo-800 to-indigo-600 text-white">Subtale OSS</span></span>,
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
        text: <span>
            Copyright © {new Date().getFullYear()} Subtale. Mímir is dual-licensed under MIT and Apache-2.0.
        </span>
    }
}
