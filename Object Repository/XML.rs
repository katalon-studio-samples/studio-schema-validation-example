<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>XML</name>
   <tag></tag>
   <elementGuidId>95697f16-6217-4bdb-846b-80824cecad63</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003cList\u003e\n  \u003citem\u003e\n    \u003cid\u003e3\n    \u003c/id\u003e\n    \u003cusername\u003eJames Johnson\n    \u003c/username\u003e\n    \u003cpassword\u003e789\n    \u003c/password\u003e\n    \u003cgender\u003eFEMALE\n    \u003c/gender\u003e\n    \u003cavatar/\u003e\n  \u003c/item\u003e\n\u003c/List\u003e\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>e7373703-5efe-4ffd-a36c-ca2e0c87eea3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.HOST}/api/users/accept-xml?gender=FEMALE&amp;age=1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>12280b78-1e45-45e5-833b-75a8697ee847</id>
      <name>right</name>
      <type>XML_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>example/xml/person.xsd</data>
      <activate>true</activate>
   </validationSteps>
   <validationSteps>
      <id>2a3b8a22-63a3-47e0-8318-fedefeaafa7d</id>
      <name>wrong</name>
      <type>AUTO_DETECT</type>
      <dataType>AUTO</dataType>
      <target>RESPONSE</target>
      <data>example/xml/personWrong.xsd</data>
      <activate>true</activate>
   </validationSteps>
   <validationSteps>
      <id>9a983cf1-55bf-4e4e-bf17-625c1874f5a3</id>
      <name>body</name>
      <type>XML_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>REQUEST_BODY</target>
      <data>example/xml/person.xsd</data>
      <activate>true</activate>
   </validationSteps>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
