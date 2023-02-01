function Subtale() {
    return <span className="ml-4 text-sm flex items-center rounded-full bg-gradient-to-tr from-[#FEAC5E] via-[#C779D0] to-[#4BC0C8]" style={{padding: "2px"}}>
        <span className="bg-white rounded-full px-3 py-1 subtale"><span className="text-transparent bg-clip-text bg-gradient-to-tr from-[#FEAC5E] via-[#C779D0] to-[#4BC0C8]">Subtale OSS</span></span>
    </span>
}

export default function Logo() {
    return <span className="font-black text-lg flex items-center">
        🧠 Mímir <Subtale />
    </span>
}