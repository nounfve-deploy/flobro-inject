import "./define"

window.ScriptInject.UserInfo = {
    id: "$__ID__",
    nickname: "$__NICKNAME__",
}

const buildUserDisplay = () => {
    const { id, nickname } = window.ScriptInject.UserInfo || {}
    const display = `${id}(${nickname || ""})`.replace("()", "")
    return display.length === 0 ? undefined : display
}

const init = () => {
    new MutationObserver((mutations, observer) => {
        for (const { type, addedNodes } of mutations) {
            if (type !== "childList" || addedNodes.length === 0) {
                continue
            }
            const input = document.querySelector<HTMLInputElement>("#usernameInput")
            if (!!input) {
                input.value = buildUserDisplay() || input.value
                return observer.disconnect()
            }
        }
    }).observe(document.documentElement, { childList: true, subtree: true })
}

if (location.href.startsWith("https://p2p.mirotalk.com/join/")) {
    document.addEventListener("DOMContentLoaded", init)
}