function Subtale() {
    return <span className="ml-4 text-sm flex items-center">
        <span className="text-white dark:text-black bg-black dark:bg-white rounded-l px-3 py-1 border border-black dark:border-white font-bold">Subtale</span>
        <span className="px-3 py-1 rounded-r border border-black dark:border-white -ml-1 font-medium">OSS</span>
    </span>
}

export default function Logo() {
    return <span className="font-black text-lg flex items-center">
        Mímir <Subtale />
    </span>
}