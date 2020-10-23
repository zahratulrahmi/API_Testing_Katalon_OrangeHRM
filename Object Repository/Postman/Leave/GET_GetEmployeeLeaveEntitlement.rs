<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_GetEmployeeLeaveEntitlement</name>
   <tag></tag>
   <elementGuidId>eaf70be1-5c47-49b0-afbe-3e7b96273c10</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${host}/api/v1/employee/14/entitlement?id=14&amp;fromDate=2021-10-01&amp;toDate=2022-09-30</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>65897ac4-4266-4f75-86b4-4fff3b7c67c1</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>aa3b1a61-eeda-4a5b-8a5c-2ff70f9f649e</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
WS.verifyElementPropertyValue(response, 'data[0].id', &quot;3&quot;)
WS.verifyElementPropertyValue(response, 'data[0].type', &quot;Cuti&quot;)
WS.verifyElementPropertyValue(response, 'data[0].validFrom', &quot;2021-10-01&quot;)
WS.verifyElementPropertyValue(response, 'data[0].validTo', &quot;2022-09-30&quot;)
WS.verifyElementPropertyValue(response, 'data[1].id', &quot;4&quot;)
WS.verifyElementPropertyValue(response, 'data[1].type', &quot;Cuti&quot;)
WS.verifyElementPropertyValue(response, 'data[1].validFrom', &quot;2021-10-01&quot;)
WS.verifyElementPropertyValue(response, 'data[1].validTo', &quot;2022-09-30&quot;)
WS.verifyElementPropertyValue(response, 'data[2].id', &quot;10&quot;)
WS.verifyElementPropertyValue(response, 'data[2].type', &quot;Cuti&quot;)
WS.verifyElementPropertyValue(response, 'data[2].validFrom', &quot;2021-10-01&quot;)
WS.verifyElementPropertyValue(response, 'data[2].validTo', &quot;2022-09-30&quot;)
WS.verifyElementPropertyValue(response, 'data[3].id', &quot;20&quot;)
WS.verifyElementPropertyValue(response, 'data[3].type', &quot;Cuti&quot;)
WS.verifyElementPropertyValue(response, 'data[3].validFrom', &quot;2021-10-01&quot;)
WS.verifyElementPropertyValue(response, 'data[3].validTo', &quot;2022-09-30&quot;)
WS.verifyElementPropertyValue(response, 'data[4].id', &quot;21&quot;)
WS.verifyElementPropertyValue(response, 'data[4].type', &quot;Cuti&quot;)
WS.verifyElementPropertyValue(response, 'data[4].validFrom', &quot;2021-10-01&quot;)
WS.verifyElementPropertyValue(response, 'data[4].validTo', &quot;2022-09-30&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
