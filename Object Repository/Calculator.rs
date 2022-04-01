<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Calculator</name>
   <tag></tag>
   <elementGuidId>8e1eb24d-6078-4d1b-ad67-e032c0620d55</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://tempuri.org/Add</value>
      <webElementGuid>91da08d0-5f47-4c16-b402-3a0f7729010a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>051c5bbe-20c8-41cd-861b-7453f5c7c4fa</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:Add>
         &lt;tem:intA>2&lt;/tem:intA>
         &lt;tem:intB>3&lt;/tem:intB>
      &lt;/tem:Add>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://www.dneonline.com/calculator.asmx</soapServiceEndpoint>
   <soapServiceFunction>Add</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <validationSteps>
      <id>e7475836-f153-4338-9d72-50cc062f1bf2</id>
      <name>right</name>
      <type>XML_SCHEMA</type>
      <dataType>URL</dataType>
      <target>RESPONSE</target>
      <data>http://www.dneonline.com/calculator.asmx?WSDL</data>
      <activate>true</activate>
      <results>
         <status>PASS</status>
      </results>
   </validationSteps>
   <validationSteps>
      <id>9f2b5df3-c155-4db9-b99b-07f9096583b3</id>
      <name>wrong</name>
      <type>XML_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>example/soap/calcWrong.wsdl</data>
      <activate>true</activate>
      <results>
         <message> Cannot find the declaration of element 'AddResponse'.</message>
         <status>FAIL</status>
      </results>
   </validationSteps>
   <validationSteps>
      <id>0874afa4-7bbf-4df8-b5a2-4d77a1d04d63</id>
      <name>xsd</name>
      <type>AUTO_DETECT</type>
      <dataType>AUTO</dataType>
      <target>RESPONSE</target>
      <data>example/soap/calc.xsd</data>
      <activate>true</activate>
      <results>
         <status>PASS</status>
      </results>
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
   <wsdlAddress>http://www.dneonline.com/calculator.asmx?WSDL</wsdlAddress>
</WebServiceRequestEntity>
