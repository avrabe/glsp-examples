/********************************************************************************
 * Copyright (c) 2022 EclipseSource and others.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License v. 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0.
 *
 * This Source Code may also be made available under the following Secondary
 * Licenses when the conditions for such availability set forth in the Eclipse
 * Public License v. 2.0 are satisfied:
 * -- GNU General Public License, version 2 with the GNU Classpath Exception
 * which is available at https://www.gnu.org/software/classpath/license.html
 * -- MIT License which is available at https://opensource.org/license/mit.
 *
 * SPDX-License-Identifier: EPL-2.0 OR GPL-2.0 WITH Classpath-exception-2.0 OR MIT
 ********************************************************************************/
import { DefaultModelState, JsonModelState } from '@eclipse-glsp/server';
import { injectable } from 'inversify';
import { TasklistModel } from '../database/interfaces/component-tasklist-tasklist.js';

@injectable()
export class TaskListModelState extends DefaultModelState implements JsonModelState<TasklistModel> {
    protected _world: TasklistModel;

    public log(msg: string): void {
        console.log('state ' + msg);
    }

    get sourceModel(): TasklistModel {
        return this._world;
    }

    updateSourceModel(taskList: TasklistModel): void {
        this._world = taskList;
        this.log(this._world.id());
    }
}
