declare global {
    interface Window {
        ScriptInject: ScriptInject
    }
}

export type ScriptInject = {
    UserInfo?: {
        id: string,
        nickname?: string,
    }
}

window.ScriptInject = {}