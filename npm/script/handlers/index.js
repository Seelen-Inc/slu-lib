"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports, p)) __createBinding(exports, m, p);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.Obtainable = Obtainable;
__exportStar(require("./invokers.js"), exports);
__exportStar(require("./events.js"), exports);
const core_1 = require("@tauri-apps/api/core");
const event_1 = require("@tauri-apps/api/event");
function Obtainable(invokeKey, eventKey) {
    return class {
        static async getAsync() {
            return await (0, core_1.invoke)(invokeKey);
        }
        static async onChange(cb) {
            return await (0, event_1.listen)(eventKey, (event) => {
                cb(event.payload);
            });
        }
    };
}