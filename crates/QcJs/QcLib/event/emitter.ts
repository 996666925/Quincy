
/**
 * 用于包装事件的一个小类
 * a little class to pack the event 
 */
export class FuncPack {
    /** 函数 function */
    public func: Function;
    /** 上下文 context */
    public context: any;

    constructor(func: Function, context: any) {
        this.func = func;
        this.context = context;
    }
}

/**
 * 用于事件管理
 * to manage event 
 */
export class Emitter<T> {
    private _messageTable: Map<T, FuncPack[]>;

    constructor() {
        this._messageTable = new Map<T, FuncPack[]>();
    }

    /**
     * 开始监听项 
     * start listening item
     * @param eventType 监听类型 listen type
     * @param handler 监听函数 listen function 
     * @param context 监听上下文 listen context
     */
    public on(eventType: T, handler: Function, context: any) {
        let list = this._messageTable.get(eventType);
        if (!list) {
            list = [];
            this._messageTable.set(eventType, list);
        }

        if (!this.has(eventType, handler)) {
            list.push(new FuncPack(handler, context));
        }
    }

    /**
     * 移除监听项
     * remove listening item
     * @param eventType 事件类型 event type
     * @param handler 事件函数 event function 
     */
    public off(eventType: T, handler: Function) {
        let messageData = this._messageTable.get(eventType);
        if (messageData) {
            let index = messageData.findIndex(data => data.func == handler);
            if (index != -1)
                messageData.splice(index, 1);
        }
    }

    /**
     * 触发该事件
     * trigger this event 
     * @param eventType 事件类型 event type
     * @param data 事件数据 event data
     */
    public emit(eventType: T, ...data: any[]) {
        let list = this._messageTable.get(eventType);
        if (list) {
            for (let observer of list) {
                observer.func.call(observer.context, ...data);
            }
        }
    }

    /**
     * 判断是否存在该类型的观察者
     * judge if the viewer of this type exist  
     * @param eventType 事件类型 event type 
     * @param handler 事件函数 event function 
     */
    public has(eventType: T, handler: Function): boolean {
        let list = this._messageTable.get(eventType);
        return list ? list.some(observer => observer.func === handler) : false;
    }
}

