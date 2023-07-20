﻿using System;
using UnityEngine;

namespace SkyHookTest
{
    public class SkyHookTest : MonoBehaviour
    {
        public SkyHook.SkyHook hook;
        
        private void Start()
        {
            hook.StartHook();
        }

        private void Update()
        {
            foreach (var ev in hook.ReadQueue())
            {
                var now = DateTime.Now;
                Debug.Log($"{ev.Time} {now}");
            }
        }
    }
}