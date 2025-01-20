import { server } from "@/modules"

const resource = 'sessions'

export default {
    register: (subscriptions: string[]): Promise<string> => {
        return new Promise((ok, err) => {
            server.post(resource, 'register', { subscriptions })
                .then(r => {
                    const clientId: string = r.data
                    ok(clientId)
                })
                .catch(e => err(e))
        })
    },

}