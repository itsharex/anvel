import { createContext } from 'react'

type GlobalContext={
    API_URL:string,
    ws:any
}

 export const GlobalContext=createContext<GlobalContext>({
    API_URL:"",
    ws:null
 })
