import { v4 as uuid } from 'uuid';
import { Dialog } from '@capacitor/dialog';
import { stringToGradient, stringToColors, stringToSmoothGradient, getKeyColor, hexToHSLO, hslToHex } from './color-tools';

export default {
    isMobile(): boolean {
        if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)) {
          return true
        } else {
          return false
        }
    },

    isSafari(): boolean {
      return navigator.userAgent.includes('Safari')
    },

    titleCase(string: string): string {
      let str = string.replace(/([^\W_]+[^\s-]*) */g, function(txt) {
        return txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase();
      });
  
      const lowers = ['A', 'An', 'The', 'And', 'But', 'Or', 'For', 'Nor', 'As', 'At', 
      'By', 'For', 'From', 'In', 'Into', 'Near', 'Of', 'On', 'Onto', 'To', 'With'];
      for (let i = 0, j = lowers.length; i < j; i++)
        str = str.replace(new RegExp('\\s' + lowers[i] + '\\s', 'g'), 
          function(txt) {
            return txt.toLowerCase();
          });
  
      const uppers = ['Id', 'Tv'];
      for (let i = 0, j = uppers.length; i < j; i++)
        str = str.replace(new RegExp('\\b' + uppers[i] + '\\b', 'g'), 
          uppers[i].toUpperCase());
  
      return str;
    },
    
    cleanDate(date: Date): string {
      return (date.getMonth() + 1) + '-' 
        + date.getDate() + '-' 
        + date.getFullYear() + ' '
        + (date.getHours() % 12 === 0 ? 12 : date.getHours() % 12) + ':' 
        + ('0' + date.getMinutes()).slice(-2) 
        + (date.getHours() > 12 ? ' PM' : ' AM' );
    },
    
    secondsToTime(e: number): string {
      if (e === 0) {
        return "0:00";
      }
      if (e === -1) {
        return "Error"
      }
      if (isNaN(e)) {
        return "0:00";
      }
      const h = Math.floor(e / 3600),
            m = Math.floor((e % 3600) / 60),
            s = Math.floor(e % 60).toString().padStart(2, '0');
  
      if (h > 0) {
        return `${h}:${m.toString().padStart(2, '0')}:${s}`;
      } else if (m > 0) {
        return `${m}:${s}`;
      } else {
        return `0:${s}`;
      }
    },
    
    strEncodeUTF8(str: string): Uint8Array {
      const buf = new ArrayBuffer(str.length * 2);
      const bufView = new Uint8Array(buf);
      for (let i = 0, strLen = str.length; i < strLen; i++) {
          bufView[i] = str.charCodeAt(i);
      }
      return new Uint8Array(buf);
    },
    
    currentDay(): string {
      const days = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
      const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'June', 'July', 'Aug', 'Sept', 'Oct', 'Nov', 'Dec'];
      const date = new Date();
      const day = days[date.getDay()];
      const month = months[date.getMonth()];
      return day + ', ' + month + ' ' + date.getDate();
    },
    
    moveInArray(arr: any[], from: number, to: number): any[] {
      const item = arr.splice(from, 1);
      arr.splice(to, 0, item[0]);
      return arr;
    },
    
    updateExtension(extension: any, key: string, value?: any): any[] {
      const newExtension: any[] = []
      Object.keys(extension).includes(key) ? newExtension[key].push(value) : newExtension.push({ key: value });
      return newExtension;
    },
    
    checkExtension(extension: any, key: string): boolean {
      return Object.keys(extension).includes(key);
    },
    
    togglePasswordVisibility() {
      const password = document.querySelector('.viewable-password')! as HTMLInputElement;
      if (password) {
        password.type = password.type === 'password' ? 'text' : 'password';
      }
    },
    
    clearAllWebData() {
        localStorage.clear();
        sessionStorage.clear();

        if (document.cookie) {
          document.cookie.split(";").forEach((c) => {
            document.cookie = c.replace(/^ +/, "").replace(/=.*/, "=;expires=" + new Date().toUTCString() + ";path=/");
          });
        }
      
        const dbNames = ['collections', 'files', 'updates', 'offline-files', 'albums', 'songs', 'recordings', 'images']; 
        dbNames.forEach(dbName => {
          indexedDB.deleteDatabase(dbName);
    
        });
      
        location.reload();
    },

    delay(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    },

    deleteAllCookies() {
        const cookies = document.cookie.split(';');
        for (let i = 0; i < cookies.length; i++) {
          const cookie = cookies[i];
          const eqPos = cookie.indexOf('=');
          const name = eqPos > -1 ? cookie.substr(0, eqPos) : cookie;
          document.cookie = name + '=;expires=Thu, 01 Jan 1970 00:00:00 GMT';
        }
    },
    uuid,
    Dialog,
    stringToGradient,
    stringToSmoothGradient,
    stringToColors,
    getKeyColor,
    hexToHSLO,
    hslToHex,
    floatToUInt(number: number, decimals: number): number {
      return Math.round(number * Math.pow(10, decimals));
    }
  };
  