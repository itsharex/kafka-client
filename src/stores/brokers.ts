import { BrokerInfo } from "@/lib/kafka";
import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useBrokers = defineStore("brokers", () => {
  const brokers = ref<BrokerInfo[]>([]);

  const brokerIdNameMap = computed<Record<string, BrokerInfo>>(() =>
    Object.fromEntries(brokers.value.map(broker => [broker.id, broker]))
  );

  const getBrokerLabel = computed(() => (brokerId: number): String => {
    const idString = brokerId.toString();
    return idString in brokerIdNameMap.value
      ? `#${idString} - ${brokerIdNameMap.value[idString].host}`
      : `#${idString}`;
  });

  function setBrokers(metadata: BrokerInfo[]) {
    brokers.value = metadata;
  }

  return { brokers, brokerIdNameMap, getBrokerLabel, setBrokers };
});
