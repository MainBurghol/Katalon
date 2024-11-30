<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>button_Sign in</name>
   <tag></tag>
   <elementGuidId>3c256be0-e972-4e22-a4bb-43b1ecec3b85</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//button[@type='submit']</value>
      </entry>
      <entry>
         <key>CSS</key>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>button</value>
      <webElementGuid>b2034040-da45-4a50-a115-9a0961b78ec6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-data</name>
      <type>Main</type>
      <value>{
            form: null,
            isProcessing: false,
            processingMessage: null,
        }</value>
      <webElementGuid>f3f6fced-1b07-4baa-a745-70f2b7610eb2</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-init</name>
      <type>Main</type>
      <value>
            form = $el.closest('form')

            form?.addEventListener('form-processing-started', (event) => {
                isProcessing = true
                processingMessage = event.detail.message
            })

            form?.addEventListener('form-processing-finished', () => {
                isProcessing = false
            })
        </value>
      <webElementGuid>66e2920c-0e21-4fda-a3ae-85ffbdc849d7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:class</name>
      <type>Main</type>
      <value>{ 'enabled:opacity-70 enabled:cursor-wait': isProcessing }</value>
      <webElementGuid>e1b414ac-8f6e-4d3e-9857-617e93db50e7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>fi-btn relative grid-flow-col items-center justify-center font-semibold outline-none transition duration-75 focus-visible:ring-2 rounded-lg fi-color-custom fi-btn-color-primary fi-color-primary fi-size-md fi-btn-size-md gap-1.5 px-3 py-2 text-sm inline-grid shadow-sm bg-custom-600 text-white hover:bg-custom-500 focus-visible:ring-custom-500/50 dark:bg-custom-500 dark:hover:bg-custom-400 dark:focus-visible:ring-custom-400/50 fi-ac-action fi-ac-btn-action</value>
      <webElementGuid>7dc2ea74-5065-46f4-a819-cbbf94669859</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>type</name>
      <type>Main</type>
      <value>submit</value>
      <webElementGuid>4fa58832-d413-4801-a3c6-42d196e1d868</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>wire:loading.attr</name>
      <type>Main</type>
      <value>disabled</value>
      <webElementGuid>e5cd313c-f7b8-4e88-bf89-b70cf8139408</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:disabled</name>
      <type>Main</type>
      <value>isProcessing</value>
      <webElementGuid>777aac43-05f3-4aea-946a-efea819504d6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            

                    
    
    

        

                    
    
    

        
    

    
        Sign in
    

            
    

    

    
</value>
      <webElementGuid>417b6934-a8bb-48b4-b329-6fdf21937867</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;form&quot;)/div[@class=&quot;fi-form-actions&quot;]/div[@class=&quot;fi-ac gap-3 grid grid-cols-[repeat(auto-fit,minmax(0,1fr))]&quot;]/button[@class=&quot;fi-btn relative grid-flow-col items-center justify-center font-semibold outline-none transition duration-75 focus-visible:ring-2 rounded-lg fi-color-custom fi-btn-color-primary fi-color-primary fi-size-md fi-btn-size-md gap-1.5 px-3 py-2 text-sm inline-grid shadow-sm bg-custom-600 text-white hover:bg-custom-500 focus-visible:ring-custom-500/50 dark:bg-custom-500 dark:hover:bg-custom-400 dark:focus-visible:ring-custom-400/50 fi-ac-action fi-ac-btn-action&quot;]</value>
      <webElementGuid>0a398161-44a2-4ac4-bab3-7893e92f414c</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//button[@type='submit']</value>
      <webElementGuid>673aad50-b444-4fd5-8cd9-7c21b1672911</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='form']/div[2]/div/button</value>
      <webElementGuid>f3405aa4-f62a-4d7d-948e-72fc40901720</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Remember me'])[1]/following::button[1]</value>
      <webElementGuid>62c6b5ed-ed23-4b9a-bb03-639b83d0153f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Hide password'])[1]/following::button[1]</value>
      <webElementGuid>a38e4d9d-a1bb-47a9-a1d6-46419de7fad8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div/button</value>
      <webElementGuid>ff2e9573-7c74-410e-b7f2-076cd84a85e7</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//button[@type = 'submit' and (text() = '
            

                    
    
    

        

                    
    
    

        
    

    
        Sign in
    

            
    

    

    
' or . = '
            

                    
    
    

        

                    
    
    

        
    

    
        Sign in
    

            
    

    

    
')]</value>
      <webElementGuid>9d1ccec6-498d-40e9-9c45-3dedde657ae1</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
