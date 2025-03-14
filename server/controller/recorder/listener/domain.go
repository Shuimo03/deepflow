/*
 * Copyright (c) 2022 Yunshan Networks
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package listener

import (
	"github.com/deepflowys/deepflow/server/controller/recorder/cache"
	"github.com/deepflowys/deepflow/server/controller/recorder/event"
	"github.com/deepflowys/deepflow/server/libs/queue"
)

type Domain struct {
	cache         *cache.Cache
	eventProducer *event.Domain
}

func NewDomain(domainLcuuid string, c *cache.Cache, eq *queue.OverwriteQueue) *Domain {
	lisener := &Domain{
		cache:         c,
		eventProducer: event.NewDomain(domainLcuuid, &c.ToolDataSet, eq),
	}
	return lisener
}

func (p *Domain) OnUpdatersCompeleted() {
	p.eventProducer.ProduceFromMySQL()
}
