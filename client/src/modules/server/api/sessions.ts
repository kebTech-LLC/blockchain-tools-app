import { server } from "@/modules"

const resource = 'sessions'

export default {
    register: (): Promise<string> => {
        return new Promise((ok, err) => {
            server.post(resource, 'register', { subscriptions: ['server-info', 'managed-position'] })
                .then(r => {
                    const clientId: string = r.data
                    ok(clientId)
                })
                .catch(e => err(e))
        })
    },

}